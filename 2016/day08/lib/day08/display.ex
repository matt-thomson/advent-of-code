defmodule Day08.Display do
  @moduledoc """
  Represents the display.
  """
  alias Day08.Display

  defstruct [:pixels, :width, :height]

  def new(width, height) do
    %Display{pixels: MapSet.new, width: width, height: height}
  end

  def pixels(%Display{pixels: pixels}), do: pixels

  def count_pixels(%Display{pixels: pixels}), do: pixels |> Enum.count

  def apply(display = %Display{pixels: pixels}, {:rect, width, height}) do
    coords = for x <- 0..(width - 1), y <- 0..(height - 1), do: {y, x}
    pixels = coords |> MapSet.new |> MapSet.union(pixels)

    %{display | pixels: pixels}
  end

  def apply(display = %Display{pixels: pixels, height: height},
            {:rotate_column, column, steps}) do
    pixels =
      pixels
      |> Enum.map(&rotate_column(&1, column, steps, height))
      |> MapSet.new

    %{display | pixels: pixels}
  end

  def apply(display = %Display{pixels: pixels, width: width},
            {:rotate_row, row, steps}) do
    pixels =
      pixels
      |> Enum.map(&rotate_row(&1, row, steps, width))
      |> MapSet.new

    %{display | pixels: pixels}
  end

  defp rotate_column({y, column}, column, steps, height) do
    {rem(y + steps, height), column}
  end
  defp rotate_column(pixel, _, _, _), do: pixel

  defp rotate_row({row, x}, row, steps, width) do
    {row, rem(x + steps, width)}
  end
  defp rotate_row(pixel, _, _, _), do: pixel
end

defimpl String.Chars, for: Day08.Display do
  alias Day08.Display

  def to_string(%Display{pixels: pixels, height: height, width: width}) do
    dots = for y <- 0..(height - 1) do
      for x <- 0..(width - 1) do
        if pixels |> MapSet.member?({y, x}), do: '■', else: ' '
      end
    end

    dots |> Enum.map(&Enum.join/1) |> Enum.join("\n")
  end
end
