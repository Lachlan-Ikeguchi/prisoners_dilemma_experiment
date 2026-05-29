-- name: CHAOS
-- random strategy that changes bias based on round number

math.randomseed(os.time())

function decide(round, history)
    local threshold = 0.5 + 0.3 * math.sin(round / 10.0)
    if math.random() < threshold then
        return "cooperate"
    else
        return "defect"
    end
end
