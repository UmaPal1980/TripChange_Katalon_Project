import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

def slurper = new groovy.json.JsonSlurper()

//--------------------------------------Search---------------------------------------------------------------------------------------
def SearchResponse = WS.sendRequestAndVerify(findTestObject('OneWayTrip/1_Search Request'))

CustomKeywords.'methods.APIKeywords.SearchCatalogueIDAndProductID'(SearchResponse)

CustomKeywords.'methods.APIKeywords.SearchToken'(SearchResponse)

//----------------------------------Price-------------------------------------------------------------------------------------------
PriceResponse = WS.sendRequestAndVerify(findTestObject('OneWayTrip/2_Price Request'))

WS.verifyResponseStatusCode(PriceResponse, 200)

//println('Value of Search Identifier Token is ' + sPriceResponse.toString())
//-------------------------------------Initiate Workbench--------------------------------------------------------------------------
def InitiateWorkBenchResponse = WS.sendRequest(findTestObject('OneWayTrip/3_TS Intiate_Workbench'))

CustomKeywords.'methods.APIKeywords.InitiateWorkBench'(InitiateWorkBenchResponse)

WS.verifyResponseStatusCode(InitiateWorkBenchResponse, 200)

//-------------------------------------Add_Offer Catalogue-----------------------------------------------------------------------------
Add_Offer_Catalogue = WS.sendRequestAndVerify(findTestObject('OneWayTrip/4_Add_Offer Catalogue'))

WS.verifyResponseStatusCode(Add_Offer_Catalogue, 200)

//-----------------------------------------Add_Traveler------------------------------------------------------------------------------
Add_Traveler = WS.sendRequestAndVerify(findTestObject('OneWayTrip/5_Add_Traveler'))

WS.verifyResponseStatusCode(Add_Traveler, 200)

//-----------------------------------------Commit--------------------------------------------------------------------------------------
def Commit = WS.sendRequest(findTestObject('OneWayTrip/6_Commit', [('reservationId') : GlobalVariable.reservationId]))

CustomKeywords.'methods.APIKeywords.Commit'(Commit)

WS.verifyResponseStatusCode(Add_Traveler, 200)

//-----------------------------------------InitiateWorkBenchWithPNR---------------------------------------------------------------------
WorkBenchWithPNR = WS.sendRequest(findTestObject('OneWayTrip/7_Intiate workbench with PNR'))

WS.verifyResponseStatusCode(WorkBenchWithPNR, 200)

def sWorkBenchWithPNR = slurper.parseText(WorkBenchWithPNR.getResponseBodyContent())

sWorkBenchIdentifierWithPNR = sWorkBenchWithPNR.ReservationResponse.Identifier.value

//println('The WorkBenchIdentifierWithPNR is ' + sWorkBenchIdentifierWithPNR)
GlobalVariable.eCacheId = sWorkBenchIdentifierWithPNR

//------------------------------------Add FOP - CreditCard---------------------------------------------


AddFoP = WS.sendRequest(findTestObject('OneWayTrip/8_Add FOP - CreditCard'))

WS.verifyResponseStatusCode(AddFoP, 200)

//----------------------------------------TAdd Payment - CreditCard--------------------------------------------------------------------------------
AddPayment = WS.sendRequestAndVerify(findTestObject('OneWayTrip/9_Add Payment - CreditCard'))

WS.verifyResponseStatusCode(AddPayment, 200)

//----------------------------------------Ticket_Issuance---------------------------------------------------------------------------------
def TicketIssuance = WS.sendRequest(findTestObject('OneWayTrip/R_Ticket Issuance'))

WS.verifyResponseStatusCode(TicketIssuance, 200)

CustomKeywords.'methods.APIKeywords.TicketNumber'(TicketIssuance)

//-----------------------------------------GetEligibility-------------------------------------------------------------------------------------
Eligibility = WS.sendRequest(findTestObject('OneWayTrip/S_GetEligibilityRequest'))

WS.verifyResponseStatusCode(Eligibility, 200)

def sEligibility = slurper.parseText(Eligibility.getResponseBodyContent())

//sChange = ((sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibilityID[0].Penalties.Change[0])['@type'])
sExchangable = sEligibility.TicketChangeEligibilityListResponse.TicketChangeEligibilityID[0].exchangeable

//println('The PNR is ' + sChange)
WS.comment('The exchangeable is ' + sExchangable)

//----------------------------------------ReservationWorkBench---------------------------------------------------------------------------------
ReservationWorkBench = WS.sendRequest(findTestObject('OneWayTrip/T_Reservationworkbench'))

WS.verifyResponseStatusCode(ReservationWorkBench, 200)

def sReservationWorkBench = slurper.parseText(ReservationWorkBench.getResponseBodyContent())

sIdentifierResWorkBench = sReservationWorkBench.ReservationResponse.Reservation.Identifier.value

GlobalVariable.IdentifierResWorkBench = sIdentifierResWorkBench

println('The IdentifierResWorkBench is ' + GlobalVariable.IdentifierResWorkBench)

//------------------------------------------SearchDomainPNR-------------------------------------------------------------------------------------
SearchDomainPNR = WS.sendRequestAndVerify(findTestObject('OneWayTrip/U-SearchDomainPNR'))

WS.verifyResponseStatusCode(SearchDomainPNR, 200)

def sSearchDomainPNR = slurper.parseText(SearchDomainPNR.getResponseBodyContent())

sExchSearchIdentifierValue = sSearchDomainPNR.CatalogOfferingsAirChangeResponse.CatalogOfferings.Identifier.value

GlobalVariable.ExchSearchIdentifierValue = sExchSearchIdentifierValue

println('The ExchSearchIdentifierValue is ' + GlobalVariable.ExchSearchIdentifierValue)

//--------------------------------------------Modify Offer----------------------------------------------------------------
ModifyOffer = WS.sendRequest(findTestObject('OneWayTrip/V_Modify Offer'))

WS.verifyResponseStatusCode(ModifyOffer, 200)

def sModifyOffer = slurper.parseText(ModifyOffer.getResponseBodyContent())

sOfferIdentifierValue = sModifyOffer.OfferListResponse.OfferID[0].Identifier.value

GlobalVariable.sOfferIdentifierValue = sOfferIdentifierValue

println('The OfferIdentifierValue is ' + GlobalVariable.sOfferIdentifierValue)

//--------------------------------------------Add Payment------------------------------------------------------------------
AddPayment = WS.sendRequest(findTestObject('OneWayTrip/X_Add Payment PNR - CreditCard'))

WS.verifyResponseStatusCode(AddPayment, 200)

//--------------------------------------------------------------------------------------------------------------------------
def ReservationCommit = WS.sendRequest(findTestObject('OneWayTrip/Y_Reservation Commit'))

CustomKeywords.'methods.APIKeywords.ReservationCommit'(ReservationCommit)

WS.verifyResponseStatusCode(ReservationCommit, 200)






