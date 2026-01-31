defmodule Yggdrasil.Application do
  @moduledoc false
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      # Service Coordinator - manages service lifecycle
      Yggdrasil.ServiceCoordinator,
      # Request Router - routes requests to services
      Yggdrasil.RequestRouter
      # TODO: Add Yggdrasil.Repo when database is set up
      # TODO: Add WebSocket handlers for Bifrost and Ratatoskr
    ]

    opts = [strategy: :one_for_one, name: Yggdrasil.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
