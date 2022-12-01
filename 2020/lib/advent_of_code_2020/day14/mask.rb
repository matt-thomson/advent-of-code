# frozen_string_literal: true

module AdventOfCode2020
  module Day14
    class Mask
      def initialize(input)
        @zeros = input.tr('X', '1').to_i(2)
        @ones = input.tr('X', '0').to_i(2)
        @floats = (0..input.length).select { |i| input[i] == 'X' }.map { |i| input.length - i - 1 }
      end

      def apply_v1(input)
        (input | @ones) & @zeros
      end

      def apply_v2(input, floats = @floats)
        return [input | @ones] if floats.empty?

        float_mask = 2**floats[0]

        apply_v2(input | float_mask, floats[1..]) + apply_v2(input & ~float_mask, floats[1..])
      end
    end
  end
end
