defmodule Day05.Hash do
  @moduledoc """
  Provides functions for working with hashes.
  """
  def hash(input, count) do
    :crypto.hash(:md5, "#{input}#{count}")
  end

  def interesting?(<<0, 0, x, _::binary>>), do: x < 16
  def interesting?(_)                     , do: false

  def password_char(<<0, 0, x, _::binary>>) do
    x |> Integer.to_string(16) |> String.downcase
  end
end
