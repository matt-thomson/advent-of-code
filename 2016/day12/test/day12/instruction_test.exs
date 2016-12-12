defmodule Day12.InstructionTest do
  use ExUnit.Case

  alias Day12.Instruction

  test "parses a copy number instruction" do
    assert "cpy 41 a" |> Instruction.parse == {:copy_number, 41, :a}
  end

  test "parses a copy register instruction" do
    assert "cpy a b" |> Instruction.parse == {:copy_register, :a, :b}
  end

  test "parses an increment instruction" do
    assert "inc a" |> Instruction.parse == {:increment, :a}
  end

  test "parses a decrement instruction" do
    assert "dec a" |> Instruction.parse == {:decrement, :a}
  end

  test "parses a jump if non-zero instruction" do
    assert "jnz a 2"  |> Instruction.parse == {:jump_if_non_zero, :a, 2}
    assert "jnz a -3" |> Instruction.parse == {:jump_if_non_zero, :a, -3}
  end

  test "parses a no-op instruction" do
    assert "jnz 0 2" |> Instruction.parse == :no_op
  end

  test "parses an unconditional jump instruction" do
    assert "jnz 1 2" |> Instruction.parse == {:jump, 2}
  end
end
