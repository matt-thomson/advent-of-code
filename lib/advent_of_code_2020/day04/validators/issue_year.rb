# frozen_string_literal: true

require_relative './year'

module AdventOfCode2020
  module Day04
    module Validators
      class IssueYear < Year
        def initialize
          super(2010, 2020)
        end
      end
    end
  end
end
