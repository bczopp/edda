defmodule Yggdrasil.TestHelpers do
  @moduledoc """
  Test utilities for Yggdrasil tests
  """

  @doc """
  Wait for a service to be ready
  """
  def wait_for_service(url, max_retries \\ 10) do
    wait_for_service(url, max_retries, 0)
  end

  defp wait_for_service(_url, max_retries, retries) when retries >= max_retries do
    false
  end

  defp wait_for_service(url, max_retries, retries) do
    case :httpc.request(:get, {String.to_charlist(url), []}, [], []) do
      {:ok, {{_, 200, _}, _, _}} -> true
      _ ->
        :timer.sleep(500)
        wait_for_service(url, max_retries, retries + 1)
    end
  end

  @doc """
  Get service URL from environment or use default
  """
  def get_service_url(service_name, default_port) do
    env_var = String.upcase(service_name) <> "_URL"
    System.get_env(env_var) || "http://localhost:#{default_port}"
  end
end
