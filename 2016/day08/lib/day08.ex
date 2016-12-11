defmodule Day08 do
  @moduledoc """
  Entry point for day 8 solver.
  """
  alias Day08.Display
  alias Day08.Instruction

  @display Display.new(50, 6)

  def part_one(filename) do
    filename
    |> File.stream!
    |> Stream.map(&Instruction.parse/1)
    |> Enum.reduce(@display, &Display.apply(&2, &1))
    |> Display.count_pixels
  end
end
