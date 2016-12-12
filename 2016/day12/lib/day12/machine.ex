defmodule Day12.Machine do
  @moduledoc """
  Represents the monorail control system.
  """
  alias Day12.Machine

  @initial_registers %{a: 0, b: 0, c: 0, d: 0}

  defstruct [:position, :registers]

  def new(registers \\ %{}) do
    registers = @initial_registers |> Map.merge(registers)
    %Machine{position: 0, registers: registers}
  end

  def position(%Machine{position: position}), do: position

  def register(%Machine{registers: registers}, register) do
    registers[register]
  end

  def run(machine, instructions) do
    instruction = instructions |> Enum.at(machine.position)

    if instruction do
      machine |> perform(instruction) |> run(instructions)
    else
      machine
    end
  end

  def perform(machine, {:copy_number, number, register}) do
    registers = machine.registers |> Map.put(register, number)
    %{machine | registers: registers, position: machine.position + 1}
  end

  def perform(machine, {:copy_register, from, to}) do
    registers = machine.registers |> Map.put(to, machine.registers[from])
    %{machine | registers: registers, position: machine.position + 1}
  end

  def perform(machine, {:increment, register}) do
    registers = machine.registers |> Map.update!(register, &(&1 + 1))
    %{machine | registers: registers, position: machine.position + 1}
  end

  def perform(machine, {:decrement, register}) do
    registers = machine.registers |> Map.update!(register, &(&1 - 1))
    %{machine | registers: registers, position: machine.position + 1}
  end

  def perform(machine, {:jump_if_non_zero, register, steps}) do
    steps = if machine.registers[register] == 0, do: 1, else: steps
    %{machine | position: machine.position + steps}
  end

  def perform(machine, :no_op) do
    %{machine | position: machine.position + 1}
  end

  def perform(machine, {:jump, steps}) do
    %{machine | position: machine.position + steps}
  end
end
