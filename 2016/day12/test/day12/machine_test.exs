defmodule Day12.MachineTest do
  use ExUnit.Case

  alias Day12.Machine

  @machine Machine.new

  test "performs a copy number instruction" do
    machine =
      @machine
      |> Machine.perform({:copy_number, 41, :a})

    assert machine |> Machine.register(:a) == 41
    assert machine |> Machine.position == 1
  end

  test "performs a copy register instruction" do
    machine =
      @machine
      |> Machine.perform({:copy_number, 41, :a})
      |> Machine.perform({:copy_register, :a, :b})

    assert machine |> Machine.register(:b) == 41
    assert machine |> Machine.position == 2
  end

  test "performs an increment instruction" do
    machine =
      @machine
      |> Machine.perform({:copy_number, 41, :a})
      |> Machine.perform({:increment, :a})

    assert machine |> Machine.register(:a) == 42
    assert machine |> Machine.position == 2
  end

  test "performs an decrement instruction" do
    machine =
      @machine
      |> Machine.perform({:copy_number, 41, :a})
      |> Machine.perform({:decrement, :a})

    assert machine |> Machine.register(:a) == 40
    assert machine |> Machine.position == 2
  end

  test "performs a jump if non-zero instruction when the register is zero" do
    machine =
      @machine
      |> Machine.perform({:jump_if_non_zero, :a, 3})

    assert machine |> Machine.position == 1
  end

  test "performs a jump if non-zero instruction when the register is non-zero" do
    machine =
      @machine
      |> Machine.perform({:increment, :a})
      |> Machine.perform({:jump_if_non_zero, :a, 3})

    assert machine |> Machine.position == 4
  end

  test "performs a no-op" do
    machine =
      @machine
      |> Machine.perform(:no_op)

    assert machine |> Machine.position == 1
  end

  test "performs a jump" do
    machine =
      @machine
      |> Machine.perform({:jump, 3})

    assert machine |> Machine.position == 3
  end
end
