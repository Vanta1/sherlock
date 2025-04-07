self: {
  lib,
  config,
  pkgs,
  ...
}: let
  inherit (lib) mkIf mkMerge mkOption types;
  cfg = config.programs.sherlock;
  aliasType = with types;
    submodule {
      options = {
        name = mkOption {
          type = str;
          default = "";
        };
        icon = mkOption {
          type = str;
          default = "";
        };
        exec = mkOption {
          type = str;
          default = "";
        };
        keywords = mkOption {
          type = str;
          default = "";
        };
      };
    };
  # TODO(Vanta_1): fix these up into proper types
  configType = types.anything;
  launcherType = types.anything;
in {
  options.programs.sherlock = with types; {
    enable = lib.mkEnableOption "Manage sherlock & config files with home-manager module." // {default = false;};

    settings = mkOption {
      description = "Sherlock settings, seperated by config file.";
      default = {};
      type = submodule {
        options = {
          aliases = mkOption {
            default = null;
            description = ''
              'sherlock_alias.json' file contents in Nix syntax, e.g.

              ```nix
              aliases.<name> = { name = "example"; };
              ```

              Would become:

              ```json
              "<name>": {
                "name": "example"
              }
              ```
            '';
            type = nullOr (attrsOf aliasType);
          };
          config = mkOption {
            default = null;
            description = ''
              `config.json` in Nix syntax.
            '';
            type = nullOr (attrsOf configType);
          };
          ignore = mkOption {
            default = "";
            description = "'sherlockignore' file contents.";
            type = nulOr lines;
          };
          launchers = mkOption {
            default = null;
            description = "'fallback.json' in Nix syntax. See ```settings.aliases``` for a similar example.";
            type = nullOr (listOf launcherType);
          };
        };
      };
    };
  };

  config =
    mkIf cfg.enable (mkMerge [
      (mkIf (cfg.settings
        != null) (mkMerge [
        (mkIf cfg.settings.aliases
          != null {
            xdg.configFile."sherlock/sherlock_alias.json".text = builtins.toJSON cfg.settings.aliases;
          })
        (mkIf cfg.settings.config
          != null {
            xdg.configFile."sherlock/config.json".text = builtins.toJSON cfg.settings.config;
          })
        (mkIf cfg.settings.ignore
          != null {
            xdg.configFile."sherlock/sherlockignore".text = cfg.settings.ignore;
          })
        (mkIf cfg.settings.fallback
          != null {
            xdg.configFile."sherlock/fallback.json".text = builtins.toJSON cfg.settings.launchers;
          })
      ]))
    ])
    // {
      # always install the package, because why else would you include the home-manager module.
      home.packages = [self.packages.${pkgs.system}.default];
    };
}
