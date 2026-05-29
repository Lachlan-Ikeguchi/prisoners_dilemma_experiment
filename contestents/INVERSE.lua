-- name: INVERSE
-- does the opposite of what opponent did last round

function decide(round, history)
    if round == 1 then
        return "cooperate"
    end
    
    if history[#history] == "cooperate" then
        return "defect"
    else
        return "cooperate"
    end
end
