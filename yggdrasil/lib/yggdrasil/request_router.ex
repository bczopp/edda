defmodule Yggdrasil.RequestRouter do
  @moduledoc """
  Request Router - Routes requests to appropriate Rust microservices
  """
  use GenServer

  def start_link(opts) do
    GenServer.start_link(__MODULE__, opts, name: __MODULE__)
  end

  def init(_opts) do
    {:ok, %{}}
  end

  def handle_call({:route_request, request_type, payload}, _from, state) do
    # Route request based on type
    result = case request_type do
      "business_request" ->
        # Route to Nornen for business logic decisions
        route_to_nornen(payload)
      
      "payment_request" ->
        # Route to Heidrun for payment processing
        route_to_heidrun(payload)
      
      "data_request" ->
        # Route to Mimir for data operations
        route_to_mimir(payload)
      
      _ ->
        {:error, "Unknown request type: #{request_type}"}
    end

    {:reply, result, state}
  end

  defp route_to_nornen(_payload) do
    # In a real implementation, this would:
    # 1. Create gRPC client to Nornen
    # 2. Send request
    # 3. Return response
    {:ok, %{provider_id: "provider-123", decision: "approved"}}
  end

  defp route_to_heidrun(_payload) do
    # In a real implementation, this would:
    # 1. Create gRPC client to Heidrun
    # 2. Send request
    # 3. Return response
    {:ok, %{price: 10.0, token_count: 100}}
  end

  defp route_to_mimir(_payload) do
    # In a real implementation, this would:
    # 1. Create gRPC client to Mimir
    # 2. Send request
    # 3. Return response
    {:ok, %{data_id: "data-123"}}
  end

  # Public API
  def route_request(request_type, payload) do
    GenServer.call(__MODULE__, {:route_request, request_type, payload})
  end
end
