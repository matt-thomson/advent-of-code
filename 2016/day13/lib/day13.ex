defmodule Day13 do
  @moduledoc """
  Entry point for day 13 solver.
  """
  @destination {31, 39}

  alias Day13.Route

  def part_one(input, destination \\ @destination) do
    input
    |> parse_num
    |> Route.new
    |> Route.find(destination)
  end

  defp parse_num(input) do
    {num, ""} = input |> Integer.parse
    num
  end
end
