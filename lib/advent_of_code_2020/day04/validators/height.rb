# frozen_string_literal: true

module AdventOfCode2020
  module Day04
    module Validators
      class Height
        HEIGHT_REGEX = /\A(?<quantity>\d+)(?<unit>cm|in)\Z/.freeze

        def valid?(value)
          matches = value.match(HEIGHT_REGEX)
          return false unless matches

          quantity = matches['quantity'].to_i

          case matches['unit']
          when 'cm' then (150..193).cover?(quantity)
          when 'in' then (59..76).cover?(quantity)
          end
        end
      end
    end
  end
end
