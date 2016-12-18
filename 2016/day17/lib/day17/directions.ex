defmodule Day17.Directions do
  @moduledoc """
  Converts directions lists to a string.
  """
  def to_string(directions), do: directions |> Enum.map(&to_char/1) |> Enum.join

  defp to_char(:up)   , do: "U"
  defp to_char(:down) , do: "D"
  defp to_char(:left) , do: "L"
  defp to_char(:right), do: "R"
end
