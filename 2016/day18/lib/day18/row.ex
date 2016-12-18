defmodule Day18.Row do
  @moduledoc """
  Functions for working with rows of tiles.
  """
  def parse(input) do
    input |> to_char_list |> Enum.map(&parse_char/1)
  end

  def step(row) do
    full_row = [:safe] ++ row ++ [:safe]

    full_row
    |> Enum.chunk(3, 1)
    |> Enum.map(&step_chunk/1)
  end

  def count_safe(row) do
    row |> Enum.count(&(&1 == :safe))
  end

  defp parse_char(?^), do: :trap
  defp parse_char(?.), do: :safe

  defp step_chunk([:trap, _, :safe]), do: :trap
  defp step_chunk([:safe, _, :trap]), do: :trap
  defp step_chunk([_, _, _]), do: :safe
end
