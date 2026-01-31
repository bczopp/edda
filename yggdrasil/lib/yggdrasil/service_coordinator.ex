defmodule Yggdrasil.ServiceCoordinator do
  @moduledoc """
  Service Coordinator - Manages lifecycle and health of all Rust microservices
  """
  use GenServer

  def start_link(opts) do
    GenServer.start_link(__MODULE__, opts, name: __MODULE__)
  end

  def init(_opts) do
    # Service endpoints configuration
    services = %{
      nidhoggr: System.get_env("NIDHOGR_ENDPOINT", "http://localhost:50061"),
      nornen: System.get_env("NORNEN_ENDPOINT", "http://localhost:50055"),
      mimir: System.get_env("MIMIR_ENDPOINT", "http://localhost:50059"),
      heidrun: System.get_env("HEIDRUN_ENDPOINT", "http://localhost:50057"),
      eikthyrnir: System.get_env("EIKTHYRNIR_ENDPOINT", "http://localhost:50063"),
      laeradr: System.get_env("LAERADR_ENDPOINT", "http://localhost:50065"),
      hirtir: System.get_env("HIRTIR_ENDPOINT", "http://localhost:50067"),
      njordr: System.get_env("NJORDR_ENDPOINT", "http://localhost:50069")
    }

    state = %{
      services: services,
      health_status: %{}
    }

    # Start health check process
    schedule_health_check()

    {:ok, state}
  end

  def handle_info(:health_check, state) do
    # Perform health checks on all services
    # For now, we'll just mark them as healthy
    # In a real implementation, we would ping each service
    
    new_health_status = 
      state.services
      |> Enum.map(fn {name, _endpoint} -> {name, :healthy} end)
      |> Enum.into(%{})

    schedule_health_check()

    {:noreply, %{state | health_status: new_health_status}}
  end

  def handle_call({:get_service_status, service_name}, _from, state) do
    status = Map.get(state.health_status, service_name, :unknown)
    {:reply, status, state}
  end

  def handle_call(:list_services, _from, state) do
    {:reply, state.services, state}
  end

  defp schedule_health_check do
    Process.send_after(self(), :health_check, 30_000) # 30 seconds
  end

  # Public API
  def get_service_status(service_name) do
    GenServer.call(__MODULE__, {:get_service_status, service_name})
  end

  def list_services do
    GenServer.call(__MODULE__, :list_services)
  end
end
