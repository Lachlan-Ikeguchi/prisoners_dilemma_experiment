-- name: CRAZED
-- completely unpredictable

math.randomseed(os.time() * round)

function decide(round, history)
    if math.random() < 0.5001 then
        return "cooperate"
    else
        return "defect"
    end
end
