defmodule Day06.Message do
  @moduledoc """
  Functions relating to handling messages.
  """
  def group_letters(input) do
    input
    |> Enum.map(&String.strip/1)
    |> Enum.map(&to_char_list/1)
    |> List.zip
    |> Enum.map(&Tuple.to_list/1)
  end

  def most_common(input) do
    {char, _} =
      input
      |> char_counts
      |> Enum.max_by(fn ({_, count}) -> count end)

    [char] |> to_char_list
  end

  def least_common(input) do
    {char, _} =
      input
      |> char_counts
      |> Enum.min_by(fn ({_, count}) -> count end)

    [char] |> to_char_list
  end

  defp char_counts(input) do
    input |> Enum.reduce(%{}, &update_count/2)
  end

  defp update_count(char, acc) do
    acc |> Map.update(char, 1, &(&1 + 1))
  end
end
