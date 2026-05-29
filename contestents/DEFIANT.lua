-- name: DEFIANT
-- refuses to be told what to do

function decide(round, history)
    if round == 1 then return "defect" end
    return history[#history] == "cooperate" and "defect" or "cooperate"
end
