defmodule Day08.Instruction do
  @moduledoc """
  Parses an instruction.
  """

  @regexes [
    ~r{(rect) (\d+)x(\d+)},
    ~r{(rotate column) x=(\d+) by (\d+)},
    ~r{(rotate row) y=(\d+) by (\d+)}
  ]

  def parse(input) do
    input = input |> String.strip

    [_, type, num1, num2] = @regexes |> Enum.find_value(&Regex.run(&1, input))

    {type |> parse_type, num1 |> parse_num, num2 |> parse_num}
  end

  defp parse_type(input) do
    input |> String.replace(" ", "_") |> String.to_atom
  end

  defp parse_num(input) do
    {num, ""} = input |> Integer.parse
    num
  end
end
