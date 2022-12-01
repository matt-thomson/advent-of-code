# frozen_string_literal: true

module AdventOfCode2020
  module Day04
    module Validators
      class EyeColour
        VALID = %w[amb blu brn gry grn hzl oth].freeze

        def valid?(value)
          VALID.include?(value)
        end
      end
    end
  end
end
