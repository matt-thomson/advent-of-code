defmodule Day07.Aba do
  @moduledoc """
  Functions to pick out ABAs from a string.
  """
  def aba([x, x, x]), do: []
  def aba([x, y, x]), do: [[x, y]]
  def aba(_),         do: []

  def abas(input) do
    input
    |> to_char_list
    |> Enum.chunk(3, 1)
    |> Enum.flat_map(&aba/1)
  end
end
