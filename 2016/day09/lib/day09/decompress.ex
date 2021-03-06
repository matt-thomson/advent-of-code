defmodule Day09.Decompress do
  @moduledoc """
  Decompresses an input string.
  """

  @regex ~r{([A-Z]*)\((\d+)x(\d+)\)(.*)}

  def step(input), do: step("", input |> String.strip)

  def full(input) do
    next = step(input)

    if next == input, do: next, else: next |> full
  end

  defp step(acc, input) do
    case input |> match do
      {start, num_chars, times, rest} ->
        repeat = rest |> String.slice(0, num_chars) |> String.duplicate(times)
        rest = rest |> String.slice(num_chars..-1)

        step(acc <> start <> repeat, rest)
      nil -> acc <> input
    end
  end

  defp match(input) do
    match = @regex |> Regex.run(input)

    case match do
      [_, start, num_chars, times, rest] ->
        {start, num_chars |> parse_int, times |> parse_int, rest}
      nil -> nil
    end
  end

  defp parse_int(input) do
    {int, ""} = input |> Integer.parse
    int
  end
end
