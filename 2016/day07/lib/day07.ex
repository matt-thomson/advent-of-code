defmodule Day07 do
  @moduledoc """
  Entry point for day 7 solver.
  """
  alias Day07.Address

  def part_one(filename) do
    filename
    |> File.stream!
    |> Stream.map(&Address.parse/1)
    |> Stream.filter(&Address.tls?/1)
    |> Enum.count
  end
end
