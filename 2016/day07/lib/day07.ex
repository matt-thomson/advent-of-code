defmodule Day07 do
  @moduledoc """
  Entry point for day 7 solver.
  """
  alias Day07.Address

  def part_one(filename) do
    filename |> solve(&Address.tls?/1)
  end

  def part_two(filename) do
    filename |> solve(&Address.ssl?/1)
  end

  defp solve(filename, filter) do
    filename
    |> File.stream!
    |> Stream.map(&Address.parse/1)
    |> Stream.filter(filter)
    |> Enum.count
  end
end
