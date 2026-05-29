-- name: CREEP
-- slowly increases defection

function decide(round, history)
    if round <= 20 then return "cooperate" end
    if round <= 40 then 
        math.randomseed(os.time())
        return math.random() < 0.7 and "cooperate" or "defect"
    end
    if round <= 60 then return "cooperate" end
    return "defect"
end
