defmodule YggdrasilTest do
  use ExUnit.Case
  doctest Yggdrasil

  test "service coordinator lists services" do
    services = Yggdrasil.ServiceCoordinator.list_services()
    assert is_map(services)
    assert Map.has_key?(services, :nornen)
    assert Map.has_key?(services, :mimir)
  end

  test "service coordinator gets service status" do
    status = Yggdrasil.ServiceCoordinator.get_service_status(:nornen)
    assert status in [:healthy, :unhealthy, :unknown]
  end

  test "request router routes business requests" do
    result = Yggdrasil.RequestRouter.route_request("business_request", %{})
    assert {:ok, _response} = result
  end

  test "request router routes payment requests" do
    result = Yggdrasil.RequestRouter.route_request("payment_request", %{})
    assert {:ok, _response} = result
  end

  test "request router routes data requests" do
    result = Yggdrasil.RequestRouter.route_request("data_request", %{})
    assert {:ok, _response} = result
  end

  test "request router handles unknown request types" do
    result = Yggdrasil.RequestRouter.route_request("unknown_type", %{})
    assert {:error, _message} = result
  end
end
