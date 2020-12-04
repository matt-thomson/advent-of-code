# frozen_string_literal: true

require_relative './year'

module AdventOfCode2020
  module Day04
    module Validators
      class BirthYear < Year
        def initialize
          super(1920, 2002)
        end
      end
    end
  end
end
