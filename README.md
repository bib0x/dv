# DVENV

This is a hacky way of organizing development environments or loading dedicated 
environment variables on demand using Nix. 

Under the hood, it's just a program  parsing a JSON representation of a Flake
and executing `nix develop <environment>`

## Usage

```
CLI for poping into Nix shell

Usage: dvenv [OPTIONS] <COMMAND>

Commands:
  list  List available environments
  run   Run a command from a targeted development environment
  use   Use a targeted development environment
  help  Print this message or the help of the given subcommand(s)

Options:
  -p, --path <DIR>  Directory containing the flake.nix file (default: DVENV_FLAKE_DIR shell variable)
  -h, --help        Print help
```

## Example

```
$ DV_FLAKE_DIR=$HOME/work/git/me/devshells dvenv  list
prod_netbox

$ DV_FLAKE_DIR=$HOME/path/to/devshells dvenv use prod_netbox
(nix-shell) $ curl -H "Authorization: Token $NETBOX_TOKEN" $NETBOX_URL/api/dcim/device-types/527/

$ DV_FLAKE_DIR=$HOME/path/to/devshells dvenv run prod_netbox 'curl -H "Authorization: Token $NETBOX_TOKEN" $NETBOX_URL/api/dcim/device-types/999/'
```

## Details

### DevShells structure

For this project, the following Nix repository has been used:

```
devshells on main 
> ls -l
total 16
drwxrwxr-x. 1 bib0x bib0x   54 Oct 13 21:32 components
-rw-r--r--. 1 bib0x bib0x  534 Oct 13 21:32 flake.lock
-rw-r--r--. 1 bib0x bib0x 5543 Dec 19 15:14 flake.nix
-rw-r--r--. 1 bib0x bib0x  411 Oct 13 21:32 README.md
drwxrwxr-x. 1 bib0x bib0x  132 Jan 13 11:09 repositories
```

`Components` are units that can be combined to create more advanced shells.
For example, a simple component would be exporting a Netbox configuration into my current bash session.

``` nix
# components/netbox.nix
{ passdir }:
{
  prod = ''
    export NETBOX_URL=$(PASSWORD_STORE_DIR=${passdir} pass netbox | sed -n -e 's/^url: \(.*\)/\1/p')
    export NETBOX_TOKEN=$(PASSWORD_STORE_DIR=${passdir} pass netbox | head -n 1)
  '';

}
```

`Repositories` are code repository with specific dependencies.
For example, if I want to use a dedicated virtualenv for an Ansible repository I can defined it such as:

``` nix
# repositories/ansible.nix
{ venvdir }:
{
  ansible = {
    pipenv = rec {
      inherit venvdir;
      name = "myproject-ansible";
      code = ''
        if [ ! -d ${venvdir}/${name} ]; then
          python3 -m venv ${venvdir}/${name}
        fi

        source ${venvdir}/${name}/bin/activate

        pip install -r requirements.txt
        ansible-galaxy install -r roles/requirements.yml
        ansible-galaxy install -r collections/requirements.yml
      '';
    };

    env = ''
      export  ANSIBLE_ACTION_WARNINGS=false
    '';

  };
}
```

`flake.nix` is where we glue the repositories, components and dependencies needed for the magic to happen.

``` nix
# flake.nix
{
  inputs.nixpkgs.url = "github:nixos/nixpkgs";

  outputs = { self, nixpkgs }:
    let 
      system = "x86_64-linux";

      # GLOBALS ------------------------------------------------------------------------------

      passdir = "$HOME/passdir";
      venvdir = "$HOME/.local/share/virtualenvs";
      
      # COMPONENTS ---------------------------------------------------------------------------
      
      netbox = import ./components/netbox.nix { inherit passdir; };
      
      # REPOSITORIES -------------------------------------------------------------------------
       
      deployment = import ./repositories/ansible.nix { inherit venvdir; };
      
    in {
    
      devShells.${system} = 
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in {
         
          # DEVENV ---------------------------------------------------------------------------
          
          prod_netbox = pkgs.mkShell {
            name = "Production Netbox";
            shellHook = ''
                ${netbox.prod}
              '';
          };
         
         # ANSIBLE ---------------------------------------------------------------------------
         
           ansible_deployment = pkgs.mkShell {
            name = "My Ansible Deployment";
            buildInputs = with pkgs; [ python311 python311Packages.pip python311Packages.netaddr ];
            shellHook = ''
              ${netbox.prod}
              ${deployment.ansible.pipenv.code}
              ${deployment.ansible.env}
            '';
          };
         
        };
    };
}
```

### Example

``` shell
$ export DV_FLAKE_DIR=$HOME/path/to/devshells
$ dvenv list
prod_netbox
ansible_deployment
```

### More

These devshells definition can also be used with `nix-direnv` for automaticly load environment when teleporting into a project directory.

``` shell
$ z ansible
direnv: loading ~/git/ansible/.envrc
direnv: using flake /home/bib0x/git/devshells#ansible_deployment
direnv: nix-direnv: Using cached dev shell
...

impure (My-Ansible-Deployment-env) $ cat .envrc
use flake ~/git/devshells#ansible_deployment
```

