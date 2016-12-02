defmodule Day02.Step do
  def step(:up, {0, col}),    do: {0, col}
  def step(:up, {1, col}),    do: {0, col}
  def step(:up, {2, col}),    do: {1, col}

  def step(:down, {0, col}),  do: {1, col}
  def step(:down, {1, col}),  do: {2, col}
  def step(:down, {2, col}),  do: {2, col}

  def step(:left, {row, 0}),  do: {row, 0}
  def step(:left, {row, 1}),  do: {row, 0}
  def step(:left, {row, 2}),  do: {row, 1}

  def step(:right, {row, 0}), do: {row, 1}
  def step(:right, {row, 1}), do: {row, 2}
  def step(:right, {row, 2}), do: {row, 2}
end
