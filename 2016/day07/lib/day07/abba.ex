defmodule Day07.Abba do
  @moduledoc """
  Functions to determine whether a string contains an ABBA.
  """
  def abba?([x, x, x, x]), do: false
  def abba?([x, y, y, x]), do: true
  def abba?(_),            do: false

  def has_abba?(input) do
    input
    |> to_char_list
    |> Enum.chunk(4, 1)
    |> Enum.any?(&abba?/1)
  end
end
