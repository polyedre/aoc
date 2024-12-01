(use-modules ((gnu packages terminals)
              #:select (alacritty))
             ((gnu packages commencement)
              #:select (gcc-toolchain))
             ((gnu packages rust)
              #:select (rust-analyzer))
             ((gnu packages crates-io)
              #:select (rust-clippy-0.0))
             ((gnu packages rust-apps)
              #:select (rust-cargo)))

(concatenate-manifests
 (list (package->development-manifest alacritty)
       (packages->manifest (list gcc-toolchain
                                 rust-cargo
                                 rust-analyzer
                                 rust-clippy-0.0))))
