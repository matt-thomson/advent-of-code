defmodule Day08.InstructionTest do
  use ExUnit.Case

  alias Day08.Instruction

  test "parses a rect instruction" do
    input = "rect 3x2"

    assert input |> Instruction.parse == {:rect, 3, 2}
  end

  test "parses a rotate column instruction" do
    input = "rotate column x=1 by 2"

    assert input |> Instruction.parse == {:rotate_column, 1, 2}
  end

  test "parses a rotate row instruction" do
    input = "rotate row y=0 by 4"

    assert input |> Instruction.parse == {:rotate_row, 0, 4}
  end
end
