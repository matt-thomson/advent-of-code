defmodule Day13 do
  @moduledoc """
  Entry point for day 13 solver.
  """
  @destination {31, 39}

  alias Day13.Route

  def part_one(input, destination \\ @destination) do
    input |> route |> Route.find(destination)
  end

  def part_two(input, destination \\ @destination) do
    input |> route |> Route.cubicles_within(50)
  end

  defp route(input) do
    {input, ""} = input |> Integer.parse
    input |> Route.new
  end
end
