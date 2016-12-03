defmodule Day02.Directions do
  @moduledoc """
  Parses directions from a line of characters.
  """
  def parse(input) do
    input |> String.split |> Enum.map(&parse_line/1)
  end

  defp parse_line(line) do
    line |> String.graphemes |> Enum.map(&parse_character/1)
  end

  defp parse_character("U"), do: :up
  defp parse_character("D"), do: :down
  defp parse_character("L"), do: :left
  defp parse_character("R"), do: :right
end
