defmodule Day13.RouteTest do
  use ExUnit.Case

  alias Day13.Route

  test "takes a step of the algorithm" do
    route = Route.new(10)
    route = route |> Route.step

    assert route |> Route.distance({1, 1}) == 0
  end

  test "takes multiple steps of the algorithm" do
    route = Route.new(10)
    route = route |> steps(10)

    assert route |> Route.distance({4, 2}) == 4
  end

  defp steps(route, n) do
    if n == 0, do: route, else: route |> Route.step |> steps(n - 1)
  end
end
