# frozen_string_literal: true

module AdventOfCode2020
  module Day14
    class Mask
      def initialize(input)
        @zeros = input.tr('X', '1').to_i(2)
        @ones = input.tr('X', '0').to_i(2)
      end

      def apply(input)
        (input | @ones) & @zeros
      end
    end
  end
end
