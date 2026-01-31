import Config

config :yggdrasil,
  grpc_port: 50000,
  ratatoskr_port: 8080,
  bifrost_port: 8081

# TODO: Uncomment when Yggdrasil.Repo is implemented (Phase 6)
# config :yggdrasil, Yggdrasil.Repo,
#   database: "yggdrasil_dev",
#   username: "postgres",
#   password: "postgres",
#   hostname: "localhost",
#   port: 5432
