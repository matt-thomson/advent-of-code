defmodule Day04 do
  @moduledoc """
  Entry point for day 4 solver.
  """
  alias Day04.Room

  def part_one(filename) do
    filename
    |> valid_rooms
    |> Stream.map(&(&1.sector_id))
    |> Enum.sum
  end

  def part_two(filename) do
    room = filename
      |> valid_rooms
      |> Enum.find(&(Room.decrypt(&1) == "northpole object storage"))

    room.sector_id
  end

  defp valid_rooms(filename) do
    filename
    |> File.stream!
    |> Stream.map(&Room.parse/1)
    |> Stream.filter(&Room.valid?/1)
  end
end
