defmodule Day12.Instruction do
  @moduledoc """
  Parses an instruction.
  """
  @copy_number      ~r{cpy (\d+) ([a-d])}
  @copy_register    ~r{cpy ([a-d]) ([a-d])}
  @increment        ~r{inc ([a-d])}
  @decrement        ~r{dec ([a-d])}
  @jump_if_non_zero ~r{jnz ([a-d]) (-?\d+)}
  @jump_numbers     ~r{jnz (-?\d+) (-?\d+)}

  def parse(input) do
    input = input |> String.strip
    parsers |> Enum.find_value(&(&1).(input))
  end

  defp parsers do
    [
      &parse_copy_number/1,
      &parse_copy_register/1,
      &parse_increment/1,
      &parse_decrement/1,
      &parse_jump_if_non_zero/1,
      &parse_jump_numbers/1
    ]
  end

  defp parse_copy_number(input) do
    result = @copy_number |> Regex.run(input)

    if result do
      [_, num, register] = result
      {:copy_number, num |> parse_num, register |> parse_register}
    end
  end

  defp parse_copy_register(input) do
    result = @copy_register |> Regex.run(input)

    if result do
      [_, from, to] = result
      {:copy_register, from |> parse_register, to |> parse_register}
    end
  end

  defp parse_increment(input) do
    result = @increment |> Regex.run(input)

    if result do
      [_, register] = result
      {:increment, register |> parse_register}
    end
  end

  defp parse_decrement(input) do
    result = @decrement |> Regex.run(input)

    if result do
      [_, register] = result
      {:decrement, register |> parse_register}
    end
  end

  defp parse_jump_if_non_zero(input) do
    result = @jump_if_non_zero |> Regex.run(input)

    if result do
      [_, register, steps] = result
      {:jump_if_non_zero, register |> parse_register, steps |> parse_num}
    end
  end

  defp parse_jump_numbers(input) do
    result = @jump_numbers |> Regex.run(input)

    if result do
      [_, num, steps] = result
      if num == "0", do: :no_op, else: {:jump, steps |> parse_num}
    end
  end

  defp parse_num(input) do
    {num, ""} = input |> Integer.parse
    num
  end

  def parse_register(input) do
    input |> String.to_atom
  end
end
