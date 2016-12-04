defmodule Day04 do
  @moduledoc """
  Entry point for day 4 solver.
  """
  alias Day04.Room

  def part_one(filename) do
    filename
    |> File.stream!
    |> Enum.map(&Room.parse/1)
    |> Enum.filter(&Room.valid?/1)
    |> Enum.map(&(&1.sector_id))
    |> Enum.sum
  end
end
