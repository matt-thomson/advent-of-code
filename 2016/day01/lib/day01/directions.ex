defmodule Day01.Directions do
  def parse(input) do
    input
    |> String.strip
    |> String.split(", ")
    |> Enum.map(&parse_direction/1)
  end

  defp parse_direction("R" <> count), do: {:right, count |> parse_count}
  defp parse_direction("L" <> count), do: {:left,  count |> parse_count}

  defp parse_count(count) do
    {result, ""} = count |> Integer.parse
    result
  end
end
