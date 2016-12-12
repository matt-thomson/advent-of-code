defmodule Day12 do
  @moduledoc """
  Entry point for day 12 solver.
  """
  alias Day12.Instruction
  alias Day12.Machine

  def part_one(filename) do
    instructions =
      filename
      |> File.stream!
      |> Stream.map(&Instruction.parse/1)
      |> Enum.to_list

    Machine.new |> Machine.run(instructions) |> Machine.register(:a)
  end
end
