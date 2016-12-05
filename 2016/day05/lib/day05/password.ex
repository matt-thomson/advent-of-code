defmodule Day05.Password do
  @moduledoc """
  Provides functions relating to passwords.
  """

  def char_one(<<0, 0, x, _::binary>>), do: x |> int_to_char

  def char_two(<<0, 0, _, x, _::binary>>), do: x |> div(16) |> int_to_char

  def position(<<0, 0, x, _::binary>>), do: x

  def set_char(characters, hash) do
    position = hash |> position

    if characters |> Enum.at(position) do
      characters
    else
      characters |> List.replace_at(position, hash |> char_two)
    end
  end

  defp int_to_char(x), do: x |> Integer.to_string(16) |> String.downcase
end
