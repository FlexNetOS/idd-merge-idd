{
  # Dev shell for the rusty-idd Cargo workspace (crates/{core,runner,tui,spec,cli}
  # → the single `rusty-idd` binary). Lives here for historical reasons (it was
  # the openspec-tui dev shell, retired in slice 8) and is the documented nix
  # entrypoint per CLAUDE.md; it builds/runs the whole workspace from the repo root.
  description = "rusty-idd development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    in
    {
      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            config.allowUnfree = true;
          };
        in
        {
          default = pkgs.mkShell {
            # Rust toolchain to build/test/lint the workspace, plus claude-code
            # (the agent CLI that `rusty-idd tui` / `rusty-idd run` drives).
            # NOTE: the Node OpenSpec CLI is no longer an input — the lifecycle
            # engine is native Rust (crates/spec). The Node CLI is only a
            # dev-time conformance oracle, invoked ad hoc via `bunx`/`npx`.
            buildInputs = [
              pkgs.cargo
              pkgs.rustc
              pkgs.rustfmt
              pkgs.clippy
              pkgs.claude-code
            ];
            shellHook = ''
              echo "rusty-idd dev shell — run from the workspace root:"
              echo "  cargo build --workspace          # build the rusty-idd binary"
              echo "  cargo run --bin rusty-idd -- --help"
              echo "  cargo run --bin rusty-idd -- tui # the former openspec-tui"
            '';
          };
        });
    };
}
