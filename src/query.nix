let
  system = "x86_64-linux"; # harcoded for now
  nixpkgs = builtins.getFlake "github:NixOS/nixpkgs";
  pkgs = nixpkgs.legacyPackages.${system};

in
builtins.mapAttrs
  (_: v:
  let r = builtins.tryEval v;
  in
  if r.success
    && builtins.isAttrs r.value
    && r.value ? type
    && r.value.type == "derivation"
  then {
    name = r.value.pname or r.value.name or null;
    version = r.value.version or null;
    description = r.value.meta.description or null;
  }
  else null
  )
  pkgs
