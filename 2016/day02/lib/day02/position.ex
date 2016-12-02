defmodule Day02.Position do
  def position({row, col}), do: (row * 3) + col + 1
end
