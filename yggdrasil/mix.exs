defmodule Yggdrasil.MixProject do
  use Mix.Project

  def project do
    [
      app: :yggdrasil,
      version: "0.1.0",
      elixir: "~> 1.15",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger],
      mod: {Yggdrasil.Application, []}
    ]
  end

  defp deps do
    [
      {:phoenix, "~> 1.7"},
      {:phoenix_pubsub, "~> 2.1"},
      {:grpc, "~> 0.6"},
      {:protobuf, "~> 0.11"},
      {:postgrex, "~> 0.17"},
      {:ecto_sql, "~> 3.10"},
      {:jason, "~> 1.4"},
      {:plug_cowboy, "~> 2.6"}
    ]
  end
end
