# frozen_string_literal: true

module AdventOfCode2020
  module Day05
    class BoardingPass
      def initialize(input)
        @input = input
      end

      def seat_id
        @input
          .tr('F', '0')
          .tr('B', '1')
          .tr('L', '0')
          .tr('R', '1')
          .to_i(2)
      end
    end
  end
end
