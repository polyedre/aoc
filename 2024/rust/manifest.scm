(use-modules (gnu packages terminals)
             (gnu packages commencement)
             (gnu packages rust)
             (gnu packages crates-io)
             (gnu packages rust-apps))

(concatenate-manifests
 (list (package->development-manifest alacritty)
       (packages->manifest (list gcc-toolchain
                                 rust-cargo
                                 rust-analyzer
                                 rust-clippy-0.0))))
