-- name: BIPOLAR
-- alternates strategy every 10 rounds

function decide(round, history)
    local phase = math.floor((round - 1) / 10) % 2
    if phase == 0 then
        return "cooperate"
    else
        return "defect"
    end
end
