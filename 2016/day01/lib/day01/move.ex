defmodule Day01.Move do
  def move({:right, steps}, {x, y, :north}), do: {x + steps, y, :east}
  def move({:left , steps}, {x, y, :north}), do: {x - steps, y, :west}

  def move({:right, steps}, {x, y, :south}), do: {x - steps, y, :west}
  def move({:left , steps}, {x, y, :south}), do: {x + steps, y, :east}

  def move({:right, steps}, {x, y, :east}),  do: {x, y - steps, :south}
  def move({:left , steps}, {x, y, :east}),  do: {x, y + steps, :north}

  def move({:right, steps}, {x, y, :west}),  do: {x, y + steps, :north}
  def move({:left , steps}, {x, y, :west}),  do: {x, y - steps, :south}
end
